// Templates
import defaultDtifTemplate from './templates/default.json';
import squareMdtifTemplate from './templates/m-square.json';
import tweetMdtifTemplate from './templates/m-tweet.json';
import sqareDtifTemplate from './templates/square.json';
import textDtifTemplate from './templates/text.json';

export default {
	dtif: {
		default: defaultDtifTemplate,
		sqare: sqareDtifTemplate,
		text: textDtifTemplate
	},
	mdtif: {
		square: squareMdtifTemplate,
		tweet: tweetMdtifTemplate
	}
};
