import { Stack, StackProps } from "aws-cdk-lib";
import * as lambda from 'aws-cdk-lib/aws-lambda';
import { join } from "path";
import { RustFunction } from 'cargo-lambda-cdk';
import { Construct } from 'constructs';
import {EventSourceMapping, StartingPosition} from "aws-cdk-lib/aws-lambda";
import {PolicyStatement} from "aws-cdk-lib/aws-iam";

export interface SEELStackProps extends StackProps {
  signatureServiceLambdaFunctionName: string;
  emailConfirmationDynamoDbStreamArn: string;
  emailLinkClickHandlerServiceUrl: string;
  emailConfirmationRequestServiceUrl: string;
  emailConfirmationRequestInternalApiKey: string;
  emailSenderAddress: string;
}

export class CdkStack extends Stack {
  constructor(scope: Construct, id: string, props: SEELStackProps) {
    super(scope, id, props);

    const lambdaHandler = new RustFunction(this, 'SendEmailEventLambdaFunction', {
      // Path to the root directory.
      manifestPath: join(__dirname, '..', '..'),
      environment: {
        "EMAIL_CONFIRMATION_DYNAMODB_STREAM_ARN": props.emailConfirmationDynamoDbStreamArn,
        "SIGNATURE_SERVICE_LAMBDA_FUNCTION_NAME":  props.signatureServiceLambdaFunctionName,
        "EMAIL_LINK_CLICK_HANDLER_SERVICE_URL": props.emailLinkClickHandlerServiceUrl,
        "EMAIL_CONFIRMATION_REQUEST_SERVICE_URL": props.emailConfirmationRequestServiceUrl,
        "EMAIL_CONFIRMATION_REQUEST_SERVICE_INTERNAL_API_KEY": props.emailConfirmationRequestInternalApiKey,
        "EMAIL_SENDER_ADDRESS": props.emailSenderAddress
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

    const targetLambda = lambda.Function.fromFunctionName(this, 'SignatureLambda',
        props.signatureServiceLambdaFunctionName
    );

    // Attach SignatureLambda permissions to the Lambda execution role
    targetLambda.grantInvoke(lambdaHandler);

    // Attach DynamoDB Stream permissions to the Lambda execution role
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

    // Attach SES permissions to the Lambda execution role
    lambdaHandler.addToRolePolicy(new PolicyStatement({
      actions: ['ses:SendEmail', 'ses:SendRawEmail'],
      resources: ['*'],
    }));

  }
}
