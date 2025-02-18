import { Stack, StackProps } from "aws-cdk-lib";
import { join } from "path";
import { RustFunction } from 'cargo-lambda-cdk';
import { Construct } from 'constructs';
import { EventSourceMapping, StartingPosition } from "aws-cdk-lib/aws-lambda";
import { PolicyStatement } from "aws-cdk-lib/aws-iam";

export class CdkStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const streamArn =
        "arn:aws:dynamodb:eu-north-1:626635435572:table/EcrsStack-EmailConfirmationLambdaTable901FE3F0-1TVBZ9FZJB2AT/stream/2025-02-18T09:37:56.493";

    const lambdaHandler = new RustFunction(this, 'TriggerCallbackEventLambdaFunction', {
      manifestPath: join(__dirname, '..', '..'),
      environment: {
        "EMAIL_CONFIRMATION_DYNAMODB_STREAM_ARN": streamArn
      }
    });

    new EventSourceMapping(this, 'UpdateConfirmationRequestEvent', {
      target: lambdaHandler,
      batchSize: 5,
      eventSourceArn: streamArn,
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
          resources: [streamArn],
        })
    );

  }
}
