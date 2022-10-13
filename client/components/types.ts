export interface IArticleMetadata {
  title: string,
  excerpt: string,
  slug: string,
  published: Date,
  tags: string[],
}

export interface INotFoundError {
  code: number,
  description: string,
  reason: string,
}

export interface IArticle {
  metadata?: IArticleMetadata,
  html?: string,
  error?: INotFoundError,
}

export const enum HeadingType {
  H1 = "h1",
  H2 = "h2",
  H3 = "h3",
  H4 = "h4",
  H5 = "h5",
  H6 = "h6",
}
