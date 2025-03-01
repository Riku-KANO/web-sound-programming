<script lang="ts">
	import { onMount } from 'svelte';
	import Konva from 'konva';
	import type { Vector2d } from 'konva/lib/types';

	let stage: Konva.Stage;
	let layer: Konva.Layer;
	let indicatorLayer: Konva.Layer;
	let sidebarLayer: Konva.Layer;
	let zoomLevel = 1;
	const interval = 20;
	const verticalInterval = interval * 5;
	const numVerticalCells = 10;
	const HEIGHT = verticalInterval * numVerticalCells;
	let isDragging = false;
	let startX = 0;
	let startY = 0;
	let offsetX = 0;
	let offsetY = 0;
	let rectX = 100;
	let rectY = 100;
	let isRectDragging = false;
	let indicatorX = 0;
	let isMoving = false;
	let animation: Konva.Animation;
	let timelineOffsetY = 0;

	const WIDTH = window.innerWidth;

	onMount(() => {
		stage = new Konva.Stage({
			container: 'container',
			width: WIDTH,
			height: HEIGHT
		});

		layer = new Konva.Layer();
		indicatorLayer = new Konva.Layer();
		stage.add(layer);
		stage.add(indicatorLayer);

		drawHeader();
		drawSidebar();
		drawGrid();
		drawRectangle();
		drawIndicator();

		stage.off('mousedown', handleMouseDown);
		stage.off('mousemove', handleMouseMove); 
		stage.off('mouseup', handleMouseUp); 
		stage.off('mouseleave', handleMouseLeave); 
		stage.on('wheel', handleWheel);

		window.addEventListener('keydown', handleKeyDown);
		window.addEventListener('resize', handleResize);
		window.addEventListener('wheel', preventDefaultScroll, { passive: false }); // Prevent default scroll
	});

	function preventDefaultScroll(event: WheelEvent) {
		event.preventDefault();
	}

	function drawSidebar() {
		sidebarLayer = new Konva.Layer();
		stage.add(sidebarLayer);

		for (let y = 0; y < stage.height(); y += verticalInterval) {
			const rect = new Konva.Rect({
				x: 0,
				y: y,
				width: interval * 2,
				height: verticalInterval,
				fill: '#c0c0c0',
				stroke: '#a0a0a0',
				strokeWidth: 1
			});
			sidebarLayer.add(rect);

			const text = new Konva.Text({
				x: 0,
				y: y,
				width: interval * 2,
				height: verticalInterval,
				text: String(y / verticalInterval + 1),
				fontSize: 16,
				fontFamily: 'Calibri',
				fill: 'black',
				align: 'center',
				verticalAlign: 'middle'
			});
			sidebarLayer.add(text);
		}

		sidebarLayer.batchDraw();
	}

	function drawHeader() {
		const headerLayer = new Konva.Layer();
		stage.add(headerLayer);

		for (let x = 0; x < stage.width(); x += interval * 4) {
			const rect = new Konva.Rect({
				x: x,
				y: 0,
				width: interval * 4,
				height: interval,
				fill: '#c0c0c0',
				stroke: '#a0a0a0',
				strokeWidth: 1
			});
			headerLayer.add(rect);

			const text = new Konva.Text({
				x: x,
				y: 0,
				width: interval * 4,
				height: interval,
				text: String(x / (interval * 4) + 1),
				fontSize: 16,
				fontFamily: 'Calibri',
				fill: 'black',
				align: 'center',
				verticalAlign: 'middle'
			});
			headerLayer.add(text);
		}

		headerLayer.batchDraw();
	}

	function drawGrid() {
		layer.destroyChildren();

		for (let x = 0; x < stage.width(); x += interval) {
			const color = Math.floor(x / (interval * 4)) % 2 === 0 ? '#f0f0f0' : '#d0d0d0';
			const rect = new Konva.Rect({
				x: x,
				y: 0,
				width: interval * 4,
				height: stage.height(),
				fill: color,
				listening: false
			});
			layer.add(rect);
		}

		for (let x = 0; x < stage.width(); x += interval) {
			const line = new Konva.Line({
				points: [x, 0, x, stage.height()],
				stroke: '#e0e0e0',
				strokeWidth: 1
			});
			layer.add(line);
		}

		for (let y = 0; y < stage.height(); y += verticalInterval) {
			// Use verticalInterval here
			const line = new Konva.Line({
				points: [0, y, stage.width(), y],
				stroke: '#e0e0e0',
				strokeWidth: 1
			});
			layer.add(line);
		}

		layer.batchDraw();
	}

	function drawRectangle() {
		const rect = new Konva.Rect({
			x: rectX,
			y: rectY,
			width: interval,
			height: verticalInterval, 
			fill: 'red',
			draggable: true
		});

		rect.on('dragmove', () => {
			rectX = Math.floor(rect.x() / interval) * interval;
			rectY = Math.floor(rect.y() / verticalInterval) * verticalInterval; // Use verticalInterval here
			rect.position({ x: rectX, y: rectY });
			layer.batchDraw();
		});

		rect.on('dragend', () => {
			rectX = Math.floor(rect.x() / interval) * interval;
			rectY = Math.floor(rect.y() / verticalInterval) * verticalInterval; // Use verticalInterval here
			rect.position({ x: rectX, y: rectY });
			layer.batchDraw();
		});

		layer.add(rect);
		layer.batchDraw();
	}

	function drawIndicator() {
		indicatorLayer.destroyChildren();

		const indicator = new Konva.Line({
			points: [indicatorX, 0, indicatorX, stage.height()],
			stroke: 'blue',
			strokeWidth: 2
		});

		indicatorLayer.add(indicator);
		indicatorLayer.batchDraw();
	}

	function zoomIn() {
		zoomLevel *= 1.2;
		stage.scale({ x: zoomLevel, y: zoomLevel });
		stage.batchDraw();
	}

	function zoomOut() {
		zoomLevel /= 1.2;
		stage.scale({ x: zoomLevel, y: zoomLevel });
		stage.batchDraw();
	}

	function handleMouseDown(event: Konva.KonvaEventObject<MouseEvent>) {
		const mouseX = (event.evt.clientX - stage.x()) / zoomLevel;
		const mouseY = (event.evt.clientY - stage.y()) / zoomLevel;

		if (
			mouseX >= rectX &&
			mouseX <= rectX + interval &&
			mouseY >= rectY &&
			mouseY <= rectY + verticalInterval 
		) {
			isRectDragging = true;
		}
	}

	function handleMouseMove(event: Konva.KonvaEventObject<MouseEvent>) {
		if (isRectDragging) {
			offsetX = event.evt.clientX - startX * zoomLevel;
			offsetY = event.evt.clientY - startY * zoomLevel;
			stage.position({ x: offsetX, y: offsetY });
			stage.batchDraw();
		}
	}

	function handleMouseUp() {
		isRectDragging = false;
	}

	function handleMouseLeave() {
		isRectDragging = false;
	}

	function handleWheel(event: Konva.KonvaEventObject<WheelEvent>) {
		if (event.evt.ctrlKey) {
			event.evt.preventDefault();
			const oldScale = stage.scaleX();
			const pointer = stage.getPointerPosition() as Vector2d;
			const mousePointTo = {
				x: (pointer.x - stage.x()) / oldScale,
				y: (pointer.y - stage.y()) / oldScale
			};

			const direction = event.evt.deltaY > 0 ? -1 : 1;
			const newScale = direction > 0 ? oldScale * 1.2 : oldScale / 1.2;

			stage.scaleX(newScale);
			const newPos = {
				x: pointer.x - mousePointTo.x * newScale,
				y: stage.y()
			};
			stage.position(newPos);
			stage.batchDraw();
		} else {
			timelineOffsetY -= event.evt.deltaY;
			const maxOffsetY = 0;
			const minOffsetY = -(verticalInterval * (numVerticalCells - 1));
			timelineOffsetY = Math.max(Math.min(timelineOffsetY, maxOffsetY), minOffsetY);
			layer.y(timelineOffsetY);
			indicatorLayer.y(timelineOffsetY);
			sidebarLayer.y(timelineOffsetY); 
			layer.batchDraw();
			indicatorLayer.batchDraw();
			sidebarLayer.batchDraw();
		}
	}

	function handleKeyDown(event: KeyboardEvent) {
		// スペースを押したときにスクロールがされないようになど
		event.preventDefault();

		if (event.code === 'Space') {
			if (isMoving) {
				isMoving = false;
				animation.stop();
			} else {
				isMoving = true;
				animation = new Konva.Animation((frame) => {
					if (!frame) return;
					indicatorX += (frame.timeDiff / 1000) * 100; // Move 100 pixels per second
					drawIndicator();
				}, indicatorLayer);
				animation.start();
			}
		}
	}

	function handleResize() {
		const newWidth = window.innerWidth;
		const newHeight = window.innerHeight;
		stage.width(newWidth);
		stage.height(newHeight);
		drawGrid();
		drawHeader();
		drawSidebar();
		drawRectangle();
		drawIndicator();
	}
</script>

<div id="container"></div>
<button onclick={zoomIn}>Zoom In</button>
<button onclick={zoomOut}>Zoom Out</button>

<style>
	#container {
		border: 1px solid #000;
		width: 100%;
		height: 100%;
	}
</style>
