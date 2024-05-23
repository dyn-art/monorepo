// Templates
import defaultDtifTemplate from './assets/templates/default.json';
import squareMdtifTemplate from './assets/templates/m-square.json';
import tempMdtifTemplate from './assets/templates/m-temp.json';
import tweetMdtifTemplate from './assets/templates/m-tweet.json';
import nestedDtifTemplate from './assets/templates/nested.json';
import sqareDtifTemplate from './assets/templates/square.json';
import tempDtifTemplate from './assets/templates/temp.json';
import textDtifTemplate from './assets/templates/text.json';
import tweetDtifTemplate from './assets/templates/tweet.json';

export default {
	dtif: {
		default: defaultDtifTemplate,
		sqare: sqareDtifTemplate,
		text: textDtifTemplate,
		tweet: tweetDtifTemplate,
		nested: nestedDtifTemplate,
		temp: tempDtifTemplate
	},
	mdtif: {
		square: squareMdtifTemplate,
		tweet: tweetMdtifTemplate,
		temp: tempMdtifTemplate
	}
};
