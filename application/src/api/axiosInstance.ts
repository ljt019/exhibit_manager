import axios from "axios";
import { isDev } from "@/lib/is-dev";

const devBaseURL = "http://localhost:3030"; // Development URL
const prodBaseURL = "http://192.168.1.66:3030"; // Production URL

// Determine the baseURL

export const baseURL = isDev === true ? devBaseURL : prodBaseURL;

export const axiosInstance = axios.create({
  baseURL: baseURL,
});
