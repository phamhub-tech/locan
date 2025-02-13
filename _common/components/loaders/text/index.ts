import { cva, type VariantProps } from 'class-variance-authority'

export { default as TextLoader } from './Text.vue'

export const textVariants = cva(
	'w-full',
	{
		variants: {
			size: {
				default: 'h-6',
				xs: 'h-4',
				sm: 'h-5',
				lg: 'h-7',
				h2: 'h-9',
			},
		},
		defaultVariants: {
			size: 'default',
		},
	},
)

export type TextLoaderVariants = VariantProps<typeof textVariants>
