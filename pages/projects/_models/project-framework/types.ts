export interface IProjectFramework {
  id: string;
  name: string;
  icon: string;
  settings?: Record<string, any>;
}

export const Frameworks: IProjectFramework[] = [
  {
    id: 'nextjs',
    name: 'Next.js',
    icon: '/icons/jsx.svg',
    settings: {
      ignorePatterns: ['.next/*', 'node_modules/*'],
      useGitignore: true
    }
  },
  {
    id: 'nuxtjs',
    name: 'Nuxt.js',
    icon: '/icons/vue.svg',
    settings: {
      ignorePatterns: ['.nuxt/*', 'node_modules/*'],
      useGitignore: true
    }
  },
  {
    id: 'flutter',
    name: 'Flutter',
    icon: '/icons/dart.svg',
    settings: {
      ignorePatterns: ['build/*', '.dart_tool/*'],
      useGitignore: true
    }
  },
  {
    id: 'other',
    name: 'Other',
    icon: '/icons/unknown.svg'
  }
];