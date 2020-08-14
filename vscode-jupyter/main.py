#%%
import pandas as pd
import seaborn as sns

#%%
# Pandasのデータフレームとしてデータをロード
df = pd.read_csv("iris.csv")
df.head()

#%%
df.hist(by="Name", column="SepalLength", sharex=True, sharey=True)

#%%
sns.pairplot(df, hue='Name')

# %%
