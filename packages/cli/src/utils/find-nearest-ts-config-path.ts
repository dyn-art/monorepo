import * as ts from 'typescript';

export function findNearestTsConfigPath(startPoint = './') {
	return ts.findConfigFile(startPoint, ts.sys.fileExists);
}
