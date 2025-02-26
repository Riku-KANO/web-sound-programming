<script lang="ts">
	import { onMount } from 'svelte';
	import Konva from 'konva';
	import type { Vector2d } from 'konva/lib/types';

	let stage: Konva.Stage;
	let layer: Konva.Layer;
	let indicatorLayer: Konva.Layer;
	let zoomLevel = 1;
	const interval = 20;
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

	const WIDTH = 800;
	const HEIGHT = 600;

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

		drawGrid();
		drawRectangle();
		drawIndicator();

		stage.on('mousedown', handleMouseDown);
		stage.on('mousemove', handleMouseMove);
		stage.on('mouseup', handleMouseUp);
		stage.on('mouseleave', handleMouseLeave);
		stage.on('wheel', handleWheel);

		window.addEventListener('keydown', handleKeyDown);
	});

	function drawGrid() {
		layer.destroyChildren();

		for (let x = 0; x < stage.width(); x += interval) {
			const line = new Konva.Line({
				points: [x, 0, x, stage.height()],
				stroke: '#e0e0e0',
				strokeWidth: 1
			});
			layer.add(line);
		}

		for (let y = 0; y < stage.height(); y += interval) {
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
			height: interval,
			fill: 'red',
			draggable: true
		});

		rect.on('dragmove', () => {
			rectX = Math.floor(rect.x() / interval) * interval;
			rectY = Math.floor(rect.y() / interval) * interval;
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
			mouseY <= rectY + interval
		) {
			isRectDragging = true;
		} else {
			isDragging = true;
			startX = (event.evt.clientX - stage.x()) / zoomLevel;
			startY = (event.evt.clientY - stage.y()) / zoomLevel;
		}
	}

	function handleMouseMove(event: Konva.KonvaEventObject<MouseEvent>) {
		if (isDragging) {
			offsetX = event.evt.clientX - startX * zoomLevel;
			offsetY = event.evt.clientY - startY * zoomLevel;
			stage.position({ x: offsetX, y: offsetY });
			stage.batchDraw();
		}
	}

	function handleMouseUp() {
		isDragging = false;
		isRectDragging = false;
	}

	function handleMouseLeave() {
		isDragging = false;
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
</script>

<div id="container"></div>
<button onclick={zoomIn}>Zoom In</button>
<button onclick={zoomOut}>Zoom Out</button>
