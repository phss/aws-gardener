#!/usr/bin/env bash
set -x
awslocal s3 mb s3://some-bucket
awslocal s3 mb s3://another-bucket
set +x