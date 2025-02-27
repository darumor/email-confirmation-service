import { join } from 'path';
import { RustFunction } from 'cargo-lambda-cdk';
import { EndpointType, LambdaRestApi } from 'aws-cdk-lib/aws-apigateway'
import { Stack, StackProps } from "aws-cdk-lib";
import { Construct } from "constructs";


export class CdkStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const lambdaHandler = new RustFunction(this, 'HandleEmailLinkClickLambdaFunction', {
      manifestPath: join(__dirname, '..', '..'),
    /*  environment: {
        "EMAIL_CONFIRMATION_REQUEST_SERVICE_DYNAMO_TABLE_NAME": ""
      }

     */
    });

    new LambdaRestApi(this, 'EmailLinkClickHandlerLambdaAPIGateway', {
      handler: lambdaHandler,
      endpointTypes: [EndpointType.REGIONAL]
    });
  }
}


