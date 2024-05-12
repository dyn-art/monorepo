// Templates
import defaultDtifTemplate from './assets/templates/default.json';
import squareMdtifTemplate from './assets/templates/m-square.json';
import tweetMdtifTemplate from './assets/templates/m-tweet.json';
import sqareDtifTemplate from './assets/templates/square.json';
import tempDtifTemplate from './assets/templates/temp.json';
import textDtifTemplate from './assets/templates/text.json';

export default {
	dtif: {
		default: defaultDtifTemplate,
		sqare: sqareDtifTemplate,
		text: textDtifTemplate,
		temp: tempDtifTemplate
	},
	mdtif: {
		square: squareMdtifTemplate,
		tweet: tweetMdtifTemplate
	}
};
