# prql-wasi
Wasi compatible wrapper for PRQL

Requires Rust nightly for now.

Running:

```
cargo wasi build --release
```

will produce `target/wasm32-wasi/release/prql-wasi.wasm` which can be run with any wasi compatible wasm runtime.

Save the following as `input.prql`

```
from employees                                # Each line transforms the previous result.
filter start_date > @2021-01-01               # Clear date syntax.
derive [                                      # `derive` adds columns / variables.
  gross_salary = salary + payroll_tax,
  gross_cost = gross_salary + benefits_cost   # Variables can use other variables.
]
filter gross_cost > 0
group [title, country] (                      # `group` runs a pipeline over each group.
  aggregate [                                 # `aggregate` reduces a column to a row.
    average salary,
    sum     salary,
    average gross_salary,
    sum     gross_salary,
    average gross_cost,
    sum_gross_cost = sum gross_cost,          # `=` sets a column name.
    ct = count,
  ]
)
sort [sum_gross_cost, -country]               # `-country` means descending order.
filter ct > 200
take 20
```

Using [wasmtime](https://wasmtime.dev):
```
$ cat input.prql| wasmtime run target/wasm32-wasi/release/prql-wasi.wasm
```

will output:

```sql
WITH table_1 AS (
  SELECT
    title,
    country,
    salary + payroll_tax + benefits_cost AS gross_cost,
    salary + payroll_tax AS gross_salary,
    salary
  FROM
    employees
  WHERE
    start_date > DATE '2021-01-01'
)
SELECT
  title,
  country,
  AVG(salary),
  SUM(salary),
  AVG(gross_salary),
  SUM(gross_salary),
  AVG(gross_cost),
  SUM(gross_cost) AS sum_gross_cost,
  COUNT(*) AS ct
FROM
  table_1
WHERE
  gross_cost > 0
GROUP BY
  title,
  country
HAVING
  COUNT(*) > 200
ORDER BY
  sum_gross_cost,
  country DESC
LIMIT
  20
```