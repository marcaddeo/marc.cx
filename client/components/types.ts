export interface MetadataInterface {
  title: string;
  clean_title?: string;
  excerpt?: string;
  slug?: string;
  link?: string;
  published: Date;
  tags: string[];
}

export interface NotFoundErrorInterface {
  code: number;
  description: string;
  reason: string;
}

export interface ArticleInterface {
  metadata?: MetadataInterface;
  html?: string;
  error?: NotFoundErrorInterface;
}

export interface ProjectInterface {
  metadata?: MetadataInterface;
  html?: string;
  error?: NotFoundErrorInterface;
}

export const enum HeadingType {
  H1 = "h1",
  H2 = "h2",
  H3 = "h3",
  H4 = "h4",
  H5 = "h5",
  H6 = "h6",
}
