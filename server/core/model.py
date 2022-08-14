import tensorflow as tf
import pandas as pd
import numpy as np

class Model:
    
    def valueNetwork(self):
        traindata = pd.read_csv (r'../data/valueNetworkTrain.csv')

        noOfTrains = traindata.shape[0]
        noOfInputFeatures = traindata.shape[1]
        print (noOfTrains, noOfInputFeatures)

    def policyNetwork(self):
        pass

    def rlModel(self):
        pass

    def predict(self):
        pass


def valueNetworkModel():
    traindata = pd.read_csv (r'../data/valueNetworkTrain.csv')
    traindataX = traindata.iloc[1:,1:-1].values
    traindataY = traindata.iloc[1:,-1:].values

    noOfTrains = traindataX.shape[0]
    noOfInputFeatures = traindataX.shape[1]
    noOfOutputFeatures = traindataY.shape[1]
    model = tf.keras.Sequential([
    tf.keras.layers.Dense((noOfInputFeatures)),
    tf.keras.layers.Dense(128, activation='relu'),
    tf.keras.layers.Dense(10, activation='relu'),
    tf.keras.layers.Dense(noOfOutputFeatures)
    ])  

    model.compile(optimizer='adam',
              loss=tf.keras.losses.BinaryCrossentropy(from_logits=True),
              metrics=['accuracy'])


    model.fit(traindataX, traindataY, epochs=10)

    probability_model = tf.keras.Sequential([model, 
                                         tf.keras.layers.Softmax()])
    
    predictData = traindataX[0].reshape(1,-1)
    print(traindataY[0])
    print(predictData.shape)
    prediction = probability_model.predict(predictData)

    print()
    print(prediction)
    

if __name__ == "__main__":
    valueNetworkModel()