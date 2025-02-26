<script module>
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import TimeLine from './TimeLinePoC.svelte';
  import { fn } from '@storybook/test';
	import TimeLinePoC from './TimeLinePoC.svelte';

  // More on how to set up stories at: https://storybook.js.org/docs/writing-stories
  const { Story } = defineMeta({
    title: 'Example/TimeLine',
    component: TimeLinePoC,
  });
</script>

<!-- PoC -->
<Story name="PoC" />
