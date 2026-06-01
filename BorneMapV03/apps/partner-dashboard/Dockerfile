FROM node:20-alpine AS builder
WORKDIR /app
COPY tsconfig.base.json ./
COPY apps/partner-dashboard/package.json ./apps/partner-dashboard/
COPY packages/auth-client/package.json ./packages/auth-client/
RUN npm install --prefix apps/partner-dashboard && npm install --prefix packages/auth-client
COPY packages/auth-client/ ./packages/auth-client/
COPY apps/partner-dashboard/ ./apps/partner-dashboard/
RUN npm run build --prefix apps/partner-dashboard

FROM nginx:alpine
COPY --from=builder /app/apps/partner-dashboard/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
