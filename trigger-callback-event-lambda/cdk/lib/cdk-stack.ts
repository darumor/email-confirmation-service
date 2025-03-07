import { Stack, StackProps } from "aws-cdk-lib";
import { join } from "path";
import { RustFunction } from 'cargo-lambda-cdk';
import { Construct } from 'constructs';
import { EventSourceMapping, StartingPosition } from "aws-cdk-lib/aws-lambda";
import { PolicyStatement } from "aws-cdk-lib/aws-iam";

export interface TCELStackProps extends StackProps {
  emailConfirmationDynamoDbStreamArn: string;
}

export class CdkStack extends Stack {
  constructor(scope: Construct, id: string, props: TCELStackProps) {
    super(scope, id, props);

    const lambdaHandler = new RustFunction(this, 'TriggerCallbackEventLambdaFunction', {
      manifestPath: join(__dirname, '..', '..'),
      environment: {
        "EMAIL_CONFIRMATION_DYNAMODB_STREAM_ARN": props.emailConfirmationDynamoDbStreamArn
      }
    });

    new EventSourceMapping(this, 'UpdateConfirmationRequestEvent', {
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
