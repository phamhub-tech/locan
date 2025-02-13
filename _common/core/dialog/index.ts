import { open } from '@tauri-apps/plugin-dialog'

export async function selectDirectory(title?: string): Promise<string | null> {
	try {
		return await open({
			directory: true,
			multiple: false,
			title,
		})
	} catch (err) {
		console.error('Failed to open directory dialog', err);
		return null;
	}
}