import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import {join} from "path";
import { RustFunction } from 'cargo-lambda-cdk';
// import * as sqs from 'aws-cdk-lib/aws-sqs';

export interface SSLStackProps extends cdk.StackProps {
  emailConfirmationLambdaArn: string;
  emailSendingLambdaArn: string;
}

export class SignatureServiceLambdaStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props: SSLStackProps) {
    super(scope, id, props);

    const lambdaHandler = new RustFunction(this, 'SignatureServiceLambdaFunction', {
      manifestPath: join(__dirname, '..', '..'),
      environment: {}
    });

    lambdaHandler.addPermission('AllowEmailConfirmationServiceLambdaInvoke', {
      principal: new cdk.aws_iam.ServicePrincipal('lambda.amazonaws.com'),
      action: 'lambda:InvokeFunction',
      sourceArn: props.emailConfirmationLambdaArn
    });

    lambdaHandler.addPermission('AllowEmailSendingLambdaInvoke', {
      principal: new cdk.aws_iam.ServicePrincipal('lambda.amazonaws.com'),
      action: 'lambda:InvokeFunction',
      sourceArn: props.emailSendingLambdaArn
    });

  }
}
