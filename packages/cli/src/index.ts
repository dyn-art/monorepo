#!/usr/bin/env node
import { Command } from 'commander';

const program = new Command();

program
	.version('0.0.1')
	.action(() => {
		console.log('Hello World');
	})
	.description('Say hello');

program.parse(process.argv);
