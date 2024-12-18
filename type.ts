export type Book = {
	chapterCount: number;
	title: string;
};

export type Verse = {
	content: string;
	url: string;
};

export type BookWithChapters = {
	chapterCount: number;
	title: string;
	chapters: Verse[];
};

export type UrlResonse = {
	ranges: {
		[key: string]: {
			verses: {
				vsID: string;
				bookNumber: number;
				chapterNumber: number;
				verseNumber: number;
				standardCitation: string;
				abbreviatedCitation: string;
				content: string;
			}[];
		};
	};
};
