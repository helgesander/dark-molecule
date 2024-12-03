FROM node:lts-alpine

WORKDIR /app

COPY package*.json ./

RUN npm install

COPY frontend .

RUN npm run build

EXPOSE 8080

# TODO