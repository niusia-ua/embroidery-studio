import { invoke } from "@tauri-apps/api/core";

export const getAppDocumentDir = () => invoke<string>("get_app_document_dir");
