FROM node:20-alpine AS builder
WORKDIR /app
COPY tsconfig.base.json ./
COPY apps/admin-dashboard/package.json ./apps/admin-dashboard/
COPY packages/auth-client/package.json ./packages/auth-client/
RUN npm install --prefix apps/admin-dashboard && npm install --prefix packages/auth-client
COPY packages/auth-client/ ./packages/auth-client/
COPY apps/admin-dashboard/ ./apps/admin-dashboard/
RUN npm run build --prefix apps/admin-dashboard

FROM nginx:alpine
COPY --from=builder /app/apps/admin-dashboard/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
