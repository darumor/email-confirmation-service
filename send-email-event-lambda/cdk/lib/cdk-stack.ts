import { Stack, StackProps } from "aws-cdk-lib";
import { join } from "path";
import { RustFunction } from 'cargo-lambda-cdk';
import { Construct } from 'constructs';
import {EventSourceMapping, StartingPosition} from "aws-cdk-lib/aws-lambda";
import {PolicyStatement} from "aws-cdk-lib/aws-iam";

export interface SEELStackProps extends StackProps {
  signatureServiceLambdaFunctionName: string;
  emailConfirmationDynamoDbStreamArn: string;
}

export class CdkStack extends Stack {
  constructor(scope: Construct, id: string, props: SEELStackProps) {
    super(scope, id, props);

    const lambdaHandler = new RustFunction(this, 'SendEmailEventLambdaFunction', {
      // Path to the root directory.
      manifestPath: join(__dirname, '..', '..'),
      environment: {
        "EMAIL_CONFIRMATION_DYNAMODB_STREAM_ARN": props.emailConfirmationDynamoDbStreamArn,
        "SIGNATURE_SERVICE_LAMBDA_FUNCTION_NAME":  props.signatureServiceLambdaFunctionName
      }
    });

    new EventSourceMapping(this, 'CreateConfirmationRequestEvent', {
      target: lambdaHandler,
      batchSize: 5,
      eventSourceArn: props.emailConfirmationDynamoDbStreamArn,
      startingPosition: StartingPosition.TRIM_HORIZON,
      bisectBatchOnError: true,
      retryAttempts: 10,
  });

    lambdaHandler.addToRolePolicy(
        new PolicyStatement({
          actions: [
            "dynamodb:DescribeStream",
            "dynamodb:GetRecords",
            "dynamodb:GetShardIterator",
            "dynamodb:ListStreams",
          ],
          resources: [props.emailConfirmationDynamoDbStreamArn],
        })
    );

  }
}
