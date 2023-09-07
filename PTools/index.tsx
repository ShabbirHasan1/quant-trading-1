import { useEvent } from '@putwallet/shared';
import { useEffect, useState } from 'react';
import { getSystemLanguage } from 'src/_app/api/language/utils';
import { getToolList } from 'src/_services/server/tool';

export const PTools = () => {
	const [toolList, setToolList] = useState([]);

	logger.log('toolList===>', toolList);

	useEvent(async () => {
		const language = getSystemLanguage();
		const toolList = await getToolList({ language });
		setToolList(toolList);
	});

	return <main className="overflow-hidden text-black">12</main>;
};
