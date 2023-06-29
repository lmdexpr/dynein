/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License").
 * You may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

pub mod util;

use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions

#[tokio::test]
async fn test_simple_query() -> Result<(), Box<dyn std::error::Error>> {
    let table_name = "table--test_simple_query";

    prepare_pk_sk_table(&table_name).await?;
    let mut c = util::setup().await?;
    let query_cmd = c.args(&["--region", "local", "--table", table_name, "query", "abc"]);
    query_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "pk   sk  attributes\nabc  1\nabc  2",
        ));

    util::cleanup(vec![table_name]).await
}

#[tokio::test]
async fn test_simple_desc_query() -> Result<(), Box<dyn std::error::Error>> {
    let table_name = "table--test_desc_simple_query";

    prepare_pk_sk_table(&table_name).await?;
    let mut c = util::setup().await?;
    let query_cmd = c.args(&[
        "--region", "local", "--table", table_name, "query", "abc", "-d",
    ]);
    query_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "pk   sk  attributes\nabc  2\nabc  1",
        ));

    util::cleanup(vec![table_name]).await
}

#[tokio::test]
async fn test_query_limit() -> Result<(), Box<dyn std::error::Error>> {
    let table_name = "table--test_query_limit";

    prepare_pk_sk_table(&table_name).await?;
    let mut c = util::setup().await?;
    let query_cmd = c.args(&[
        "--region", "local", "--table", table_name, "query", "abc", "-l", "1",
    ]);
    query_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("pk   sk  attributes\nabc  1"));

    util::cleanup(vec![table_name]).await
}

async fn prepare_pk_sk_table(table_name: &&str) -> Result<(), Box<dyn std::error::Error>> {
    let mut c = util::setup().await?;
    c.args(&[
        "--region", "local", "admin", "create", "table", table_name, "--keys", "pk,S", "sk,N",
    ])
    .output()?;
    let mut c = util::setup().await?;
    c.args(&[
        "--region", "local", "--table", table_name, "put", "abc", "1",
    ])
    .output()?;
    let mut c = util::setup().await?;
    c.args(&[
        "--region", "local", "--table", table_name, "put", "abc", "2",
    ])
    .output()?;
    Ok(())
}