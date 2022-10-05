FROM node:18-alpine
WORKDIR /app

COPY package.json package-lock.json ./
RUN npm install

COPY . ./

RUN npm run build


FROM node:18-alpine
WORKDIR /app

COPY --from=0 /app/package.json /app/package-lock.json /app
RUN npm install --omit=dev

COPY --from=0 /app/build /app

EXPOSE 3000

CMD ["node", "index.js"]