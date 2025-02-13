import { z } from 'zod'

interface IFileSchemaProps {
	requiredMsg?: string;
	invalidMsg?: string;
}

export const fileSchema = (options?: IFileSchemaProps) => {
	return z
		.custom<FileList | undefined>()
		.transform((files) => !!files && files.item(0))
		.refine((files) => !!files, options?.requiredMsg ?? "fileRequired");

};

export const imageSchema = (options?: IFileSchemaProps) =>
	fileSchema({ requiredMsg: options?.requiredMsg }).refine(
		(files) => !!files && files.type?.startsWith("image"),
		options?.invalidMsg ?? "invalidImage",
	);
