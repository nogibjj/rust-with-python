import polars as pl

# Load the iris dataset
df = pl.read_csv("src/data/iris.csv")

# Filter the DataFrame
filtered_df = df.filter(pl.col("sepal_length") > 5)

# Group the DataFrame and compute the sum of each column
result = (
    filtered_df.groupby("species")
    .agg(
        [
            pl.col("sepal_length").sum().alias("sepal_length"),
            pl.col("sepal_width").sum().alias("sepal_width"),
            pl.col("petal_length").sum().alias("petal_length"),
            pl.col("petal_width").sum().alias("petal_width"),
        ]
    )
    .to_pandas()
)

# Print the resulting DataFrame
print(result)