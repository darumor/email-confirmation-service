#!/opt/homebrew/opt/node/bin/node
import * as cdk from 'aws-cdk-lib';
import { CdkStack } from '../lib/cdk-stack';

const emailConfirmationDynamoDbStreamArnFromEnv = process.env.EMAIL_CONFIRMATION_DYNAMODB_STREAM_ARN || "default-value";
const signatureServiceLambdaFunctionNameFromEnv = process.env.SIGNATURE_SERVICE_LAMBDA_FUNCTION_NAME || "default-value";
const emailConfirmationRequestServiceUrlFromEnv = process.env.EMAIL_CONFIRMATION_REQUEST_SERVICE_URL || "default-value";
const emailConfirmationRequestInternalApiKeyFromEnv = process.env.EMAIL_CONFIRMATION_REQUEST_SERVICE_INTERNAL_API_KEY || "default-value";


const app = new cdk.App();
new CdkStack(app, 'EcsTcelStack', {

    emailConfirmationDynamoDbStreamArn: emailConfirmationDynamoDbStreamArnFromEnv,
    signatureServiceLambdaFunctionName: signatureServiceLambdaFunctionNameFromEnv,
    emailConfirmationRequestServiceUrl: emailConfirmationRequestServiceUrlFromEnv,
    emailConfirmationRequestInternalApiKey: emailConfirmationRequestInternalApiKeyFromEnv

    /* If you don't specify 'env', this stack will be environment-agnostic.
     * Account/Region-dependent features and context lookups will not work,
     * but a single synthesized template can be deployed anywhere. */

  /* Uncomment the next line to specialize this stack for the AWS Account
   * and Region that are implied by the current CLI configuration. */
  // env: { account: process.env.CDK_DEFAULT_ACCOUNT, region: process.env.CDK_DEFAULT_REGION },

  /* Uncomment the next line if you know exactly what Account and Region you
   * want to deploy the stack to. */
  // env: { account: '123456789012', region: 'us-east-1' },

  /* For more information, see https://docs.aws.amazon.com/cdk/latest/guide/environments.html */
});