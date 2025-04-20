import pandas as pd
import numpy as np

import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader, Dataset, random_split

from torchvision import models, datasets, transforms


# Hyperparameters
EPOCH = 5
BATCH = 32
LR = 1e-4

# ImageNet Transform
transform = transforms.Compose([
    transforms.Resize((224, 224)),
    transforms.ToTensor(),
    transforms.Normalize(mean=[0.485, 0.456, 0.406], std=[0.229, 0.224, 0.225])
])

# Load the dataset
dataset = datasets.ImageFolder('Dataset', transform=transform)
train_dataset, valid_dataset, test_dataset = random_split(dataset, [0.7, 0.15, 0.15])

# Create the DataLoaders
train_loader = DataLoader(train_dataset, batch_size=BATCH, shuffle=True)
valid_loader = DataLoader(valid_dataset, batch_size=BATCH, shuffle=True)
test_loader = DataLoader(test_dataset, batch_size=BATCH, shuffle=True)


# Set device
device = 'cuda' if torch.cuda.is_available() else 'cpu'

# Load ResNet18 model with pre-trained weights
model = models.resnet18(weights=models.ResNet18_Weights.IMAGENET1K_V1)

# Modify the final layer to match the number of classes in dataset (2)
model.fc = nn.Linear(model.fc.in_features, 2)

# Freeze the parameters of the pre-trained layers
for param in model.parameters():
    param.requires_grad = False

# Unfreeze the parameters of the last few layers for fine-tuning
for param in model.layer4.parameters():
    param.requires_grad = True


model.to(device)
criterion = nn.CrossEntropyLoss().to(device)
optimizer = optim.AdamW(model.parameters(), lr=LR)

# Training and Validation loop
for epoch in range(EPOCH):
    train_loss = 0.0
    valid_loss = 0.0
    correct = 0

    model.train()
    for i, datum in enumerate(train_loader):
        inputs, labels = datum[0].to(device), datum[1].to(device)

        optimizer.zero_grad()
        outputs = model(inputs)

        loss = criterion(outputs, labels)
        loss.backward()

        optimizer.step()

        train_loss += loss.item() * inputs.size(0)

    model.eval()
    with torch.no_grad():
        for i, datum in enumerate(valid_loader):
            inputs, labels = datum[0].to(device), datum[1].to(device)

            outputs = model(inputs)
            loss = criterion(outputs, labels)

            valid_loss += loss.item() * inputs.size(0)
            predicted = torch.argmax(outputs, dim=1)

            correct += (predicted == labels).sum().item()

    avg_train_loss = train_loss / len(train_loader.sampler)
    avg_valid_loss = valid_loss / len(valid_loader.sampler)
    
    # Print the average loss and accuracy for the epoch
    print(f"=== Epoch {epoch} ===")
    print(f"Average Training Loss: {avg_train_loss}")
    print(f"Average Validation Loss: {avg_valid_loss}")
    print(f"Validation Accuracy: {correct / len(valid_loader.sampler)}")
    

# Test the model against the test set
test_loss = 0.0
correct = 0
model.eval()
for i, datum in enumerate(test_loader):
    inputs, labels = datum[0].to(device), datum[1].to(device)
    
    outputs = model(inputs)
    loss = criterion(outputs, labels)

    test_loss += loss.item() * inputs.size(0)
    predicted = torch.argmax(outputs, dim=1)

    correct += (predicted == labels).sum().item()

# Print the average loss and accuracy for the test set
print("=== TEST ===")
avg_test_loss = test_loss / len(test_loader.sampler)
print(f"Test Loss: {avg_test_loss}")
print(f"Test Accuracy: {correct / len(test_loader.sampler)}")


# Save the model
model.to('cpu')
example = torch.rand(1, 3, 224, 224)

traced_script_module = torch.jit.trace(model, example)
traced_script_module.save("model.pt")