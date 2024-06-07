import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import reactRefresh from "@vitejs/plugin-react-refresh";
import tailwindcss from "tailwindcss";
import autoprefixer from "autoprefixer";
import fs from "fs";
import path from "path";
import dotenv from "dotenv";

dotenv.config({ path: path.resolve(__dirname, "../backend/.env") });

export default defineConfig({
  plugins: [react(), reactRefresh()],
  css: {
    postcss: {
      plugins: [tailwindcss(), autoprefixer()],
    },
  },
  server: {
    https: {
      key: fs.readFileSync(path.resolve(__dirname, "../backend/cert/key.pem")),
      cert: fs.readFileSync(
        path.resolve(__dirname, "../backend/cert/cert.pem"),
      ),
      passphrase: process.env.SSL_PASSWORD,
    },
  },
});
