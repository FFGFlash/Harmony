FROM node:24-alpine AS base
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
COPY . /app
WORKDIR /app

FROM base AS builder
ARG PUBLIC_API_URL
ARG PUBLIC_WS_URL
RUN echo "PUBLIC_API_URL=${PUBLIC_API_URL}" > .env && \
  echo "PUBLIC_WS_URL=${PUBLIC_WS_URL}" >> .env
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
RUN pnpm run build

FROM nginx:alpine
RUN rm /etc/nginx/conf.d/default.conf
COPY --from=builder /app/build /usr/share/nginx/html
COPY frontend.nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]