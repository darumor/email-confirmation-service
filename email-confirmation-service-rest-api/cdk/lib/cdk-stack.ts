import {join} from 'path';
import {RustFunction} from 'cargo-lambda-cdk';
import {EndpointType, LambdaRestApi} from 'aws-cdk-lib/aws-apigateway'
import {AttributeType, BillingMode, Table, StreamViewType} from 'aws-cdk-lib/aws-dynamodb';
import {RemovalPolicy, Stack, StackProps} from "aws-cdk-lib";
import {Construct} from "constructs";


export class CdkStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const dynamoTable = new Table(this, 'EmailConfirmationLambdaTable', {
      partitionKey: { name: 'pk', type: AttributeType.STRING },
      stream: StreamViewType.NEW_AND_OLD_IMAGES,
      // sortKey: { name: 'expires_at', type: AttributeType.NUMBER },
      billingMode: BillingMode.PAY_PER_REQUEST,
      removalPolicy: RemovalPolicy.DESTROY,
    });

    const lambdaHandler = new RustFunction(this, 'EmailConfirmationLambdaFunction', {
      // Path to the root directory.
      manifestPath: join(__dirname, '..', '..'),
      environment: {
        "EMAIL_CONFIRMATION_REQUEST_SERVICE_DYNAMO_TABLE_NAME": dynamoTable.tableName
      }
    });

    dynamoTable.grantFullAccess(lambdaHandler);


    new LambdaRestApi(this, 'EmailConfirmationLambdaAPIGateway', {
      handler: lambdaHandler,
      endpointTypes: [EndpointType.REGIONAL]
    });
  }
}


