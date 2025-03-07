#!/opt/homebrew/opt/node/bin/node
import * as cdk from 'aws-cdk-lib';
import {SignatureServiceLambdaStack} from '../lib/cdk-stack';

const emailConfirmationLambdaFromEnv = process.env.EMAIL_CONFIRMATION_LAMBDA_ARN || "default-value";
const emailSendingLambdaFromEnv = process.env.EMAIL_SENDING_LAMBDA_ARN || "default-value";

const app = new cdk.App();
new SignatureServiceLambdaStack(app, 'EcsSslStack', {

    //emailConfirmationServiceLambda: 'arn:aws:lambda:eu-north-1:626635435572:function:EcrsStack-EmailConfirmationLambdaFunctionC6B7D8BB-HKqpRjYoEeHb',
    emailConfirmationLambdaArn: emailConfirmationLambdaFromEnv,
    //emailSendingLambda: 'arn:aws:lambda:eu-north-1:626635435572:function:EcsSeelStack-SendEmailEventLambdaFunctionFF1041EF-FBnFezC9N4W4'
    emailSendingLambdaArn: emailSendingLambdaFromEnv

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