import pandas as pd

# Load the Iris dataset from the CSV file
df = pd.read_csv("src/data/iris.csv")

# Filter the rows where the column 'sepal_length' is greater than 5.0
filtered_df = df[df["sepal_length"] > 5]

# Group by the 'species' column and sum the other columns for each group
grouped_df = filtered_df.groupby("species").sum().reset_index()

print(grouped_df)
