import { path } from "@tauri-apps/api";
import { documentDir } from "@tauri-apps/api/path";

export async function studioDocumentDir() {
  const documentDirPath = await documentDir();
  return path.join(documentDirPath, "Embroidery Studio");
}
