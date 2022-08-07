import tensorflow as tf
import pandas as pd

class Model:
    
    def valueNetwork(self):
        df = pd.read_csv (r'../data/valueNetworkTrain.csv')
        print (df)

    def policyNetwork(self):
        pass

    def rlModel(self):
        pass

    def predict(self):
        pass


def valueNetwork():
    df = pd.read_csv (r'../data/valueNetworkTrain.csv')
    print (df)

if __name__ == "__main__":
    valueNetwork()