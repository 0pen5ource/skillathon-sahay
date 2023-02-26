# Use an official Node.js runtime as the parent image
FROM node:19-alpine

# Set the working directory to /app
WORKDIR /app

# Copy the package.json and package-lock.json files to the container
COPY package*.json ./

# Install any necessary dependencies
RUN npm install

# Copy the application code to the container
COPY . ./

# Build the Svelte application
RUN npm run build

# Expose port 3000 for the Node server
EXPOSE 3000

# Start the Node server
CMD ["node", "build"]
