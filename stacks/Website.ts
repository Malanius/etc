import {
  StackContext,
  StaticSite,
  StaticSiteDomainProps,
} from 'sst/constructs';

export function Website({ stack }: StackContext) {
  const customDomain: StaticSiteDomainProps | undefined =
    stack.stage === 'prod'
      ? {
          domainName: 'etc.malanius.dev',
          hostedZone: 'malanius.dev',
        }
      : undefined;

  new StaticSite(stack, 'web', {
    path: 'packages/frontend',
    buildOutput: 'dist',
    buildCommand: 'rm -r dist/ && dx build --release',
    customDomain,
    waitForInvalidation: stack.stage === 'prod',
  });
}
