FROM node:20-alpine AS builder
WORKDIR /app
COPY tsconfig.base.json ./
COPY apps/driver-web/package.json ./apps/driver-web/
COPY packages/auth-client/package.json ./packages/auth-client/
RUN npm install --prefix apps/driver-web && npm install --prefix packages/auth-client
COPY packages/auth-client/ ./packages/auth-client/
COPY apps/driver-web/ ./apps/driver-web/
RUN npm run build --prefix apps/driver-web

FROM nginx:alpine
COPY --from=builder /app/apps/driver-web/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
