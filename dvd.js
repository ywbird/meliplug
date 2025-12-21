function init() {
	const power = document.getElementById("power-btn")
	power.addEventListener("click", ()=>location.reload())

	const container = document.getElementById("monitor")
	const canvas = document.createElement("canvas")
	const ctx = canvas.getContext("2d")

	const WIDTH = 800
	const HEIGHT = 600

	const DIAGONAL = Math.sqrt((WIDTH / 2) ** 2 + (HEIGHT / 2) ** 2) + 100
	canvas.width = WIDTH + 80
	canvas.height = HEIGHT + 80

	let scanlines = new Image()
	scanlines.crossOrigin = "anonymous" // Enable CORS request
	scanlines.src = 'scanline.png' // Example scanline texture
	let dvdlogo = new Image()
	dvdlogo.crossOrigin = "anonymous" // Enable CORS request
	dvdlogo.src = 'dvdlogo.png' // Example scanline texture
	// container.appendChild(canvas)
	let fxCanvas
	try {
		fxCanvas = fx.canvas()
	}
	catch (e) {
		console.error('WebGL not supported', e)
		return
	}
	const texture = fxCanvas.texture(canvas)
	// canvas.style.display = "none"
	// canvas.parentNode.insertBefore(fxCanvas, canvas)
	container.appendChild(fxCanvas)

	const d = (a, b) => Math.sqrt(a * a + b * b)

	let icon = {
		x: 800 - 200 - 75,
		y: 600 - 200 - 50,
		dirX: 120,
		dirY: 120,
	}

	// const targetFPS = 30
	// const frameDuration = 1000 / targetFPS // duration of one frame in ms
	let lastTime = 0
	function frame() {
		if (document.hasFocus()) {

			const currentTime = Date.now()
			if (!lastTime) lastTime = currentTime
			const dt = (currentTime - lastTime) / 1000

			icon.x += icon.dirX * dt
			icon.y += icon.dirY * dt

			if (icon.x > WIDTH - 75 || icon.x < 75) {
				icon.x -= icon.dirX * dt
				icon.dirX *= -1

				const t = (Math.random() - 0.5) 
				const c = Math.cos(t)
				const s = Math.sin(t)

				const x = c * icon.dirX - s * icon.dirY
				const y = s * icon.dirX + c * icon.dirY

				icon.dirX = x
				icon.dirY = y
			}
			if (icon.y > HEIGHT - 50 || icon.y < 50) {
				icon.y -= icon.dirY * dt
				icon.dirY *= -1

				const t = (Math.random() - 0.5) 
				const c = Math.cos(t)
				const s = Math.sin(t)

				const x = c * icon.dirX - s * icon.dirY
				const y = s * icon.dirX + c * icon.dirY

				icon.dirX = x
				icon.dirY = y
			}

			if (
				(icon.dirX > 0 && icon.dirY > 0 && (
					Math.abs((icon.y - HEIGHT + 50) / (icon.x - WIDTH + 75) - (icon.dirY / icon.dirX)) < 0.0001
				) && d((icon.y - HEIGHT + 50), (icon.x - WIDTH + 75)) < 100) ||
				(icon.dirX > 0 && icon.dirY < 0 && (
					Math.abs((icon.y - 50) / (icon.x - WIDTH + 75) - (icon.dirY / icon.dirX)) < 0.0001
				) && d((icon.y - 50), (icon.x - WIDTH + 75)) < 100) ||
				(icon.dirX < 0 && icon.dirY > 0 && (
					Math.abs((icon.y - HEIGHT + 50) / (icon.x - 75) - (icon.dirY / icon.dirX)) < 0.0001
				) && d((icon.y - HEIGHT + 50), (icon.x - 75)) < 100) ||
				(icon.dirX < 0 && icon.dirY < 0 && (
					Math.abs((icon.y - 50) / (icon.x - 75) - (icon.dirY / icon.dirX)) < 0.0001
				) && d((icon.y - 50), (icon.x - 75)) < 100)
			) {
				const t = (Math.random() - 0.5) * 2
				const c = Math.cos(t)
				const s = Math.sin(t)

				const x = c * icon.dirX - s * icon.dirY
				const y = s * icon.dirX + c * icon.dirY

				icon.dirX = x
				icon.dirY = y
			}

			ctx.fillStyle = "#1a1a1a"
			ctx.fillRect(40, 40, WIDTH, HEIGHT)

			ctx.fillStyle = `hsl(${currentTime % 10000 / 10000}turn 100% 50%)`
			ctx.fillRect(40 + icon.x - 75, 40 + icon.y - 50, 150, 100)
			ctx.drawImage(dvdlogo, 40 + icon.x - 75, 40 + icon.y - 50, 150, 100)

			CRTize()

			lastTime = currentTime
		}
		requestAnimationFrame(frame)
	}

	requestAnimationFrame(frame)


	function CRTize() {
		ctx.drawImage(scanlines, 40, 40, WIDTH, HEIGHT)
		// Apply bulge pinch effect from center with WebGL
		texture.loadContentsOf(canvas)
		fxCanvas.draw(texture)
			.bulgePinch(canvas.width / 2, canvas.height / 2, DIAGONAL, 0.2)
			// .vignette(0.25, 0.74)
			.update()
	}
}
window.onload = init
