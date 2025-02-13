<template>
	<div :class="{ 'flex gap-x-2 items-center': isLong }">
		<svg
			:width="size"
			:height="size"
			:class="{ loading: isLoading }"
			viewBox="0 0 213 213"
			fill="none"
			xmlns="http://www.w3.org/2000/svg"
		>
			<path
				d="M68.3663 49.1439L103.015 14.4957C105.162 12.3478 108.645 12.3478 110.793 14.4957L145.441 49.1439C147.589 51.2918 147.589 54.7742 145.441 56.9221L110.793 91.5703C108.645 93.7182 105.162 93.7182 103.015 91.5703L68.3663 56.9221C66.2184 54.7742 66.2184 51.2918 68.3663 49.1439Z"
				stroke="#40BF6B"
				stroke-width="5"
				style="animation-delay: 900ms"
			/>
			<path
				d="M13.5655 111.723C10.4413 108.599 10.4413 103.533 13.5655 100.409L48.2137 65.7609C51.3379 62.6367 56.4032 62.6367 59.5274 65.7609L94.1757 100.409C97.2999 103.533 97.2999 108.599 94.1757 111.723L59.5274 146.371C56.4032 149.495 51.3379 149.495 48.2137 146.371L13.5655 111.723Z"
				fill="#40BF6B"
				style="animation-delay: 700ms"
			/>
			<path
				d="M119.632 111.723C116.507 108.599 116.507 103.533 119.632 100.409L154.28 65.7609C157.404 62.6367 162.469 62.6367 165.594 65.7609L200.242 100.409C203.366 103.533 203.366 108.599 200.242 111.723L165.594 146.371C162.469 149.495 157.404 149.495 154.28 146.371L119.632 111.723Z"
				fill="#40BF6B"
				style="animation-delay: 700ms"
			/>

			<g>
				<path
					d="M88.5189 137.179C86.9568 135.617 86.9568 133.084 88.5189 131.522L104.075 115.965C105.637 114.403 108.17 114.403 109.732 115.965L125.288 131.522C126.851 133.084 126.851 135.617 125.288 137.179L109.732 152.735C108.17 154.297 105.637 154.297 104.075 152.735L88.5189 137.179Z"
					fill="#CD24DB"
					style="animation-delay: 500ms"
				/>
				<path
					d="M63.7701 161.927C62.208 160.365 62.208 157.833 63.7701 156.271L79.3265 140.714C80.8886 139.152 83.4212 139.152 84.9833 140.714L100.54 156.271C102.102 157.833 102.102 160.365 100.54 161.927L84.9833 177.484C83.4212 179.046 80.8886 179.046 79.3265 177.484L63.7701 161.927Z"
					fill="#CD24DB"
					style="animation-delay: 300ms"
				/>
				<path
					d="M113.268 161.927C111.706 160.365 111.706 157.833 113.268 156.271L128.824 140.714C130.386 139.152 132.919 139.152 134.481 140.714L150.037 156.271C151.599 157.833 151.599 160.365 150.037 161.927L134.481 177.484C132.919 179.046 130.386 179.046 128.824 177.484L113.268 161.927Z"
					fill="#CD24DB"
					style="animation-delay: 300ms"
				/>
				<path
					d="M110.904 169.231C110.904 167.022 109.113 165.231 106.904 165.231C104.695 165.231 102.904 167.022 102.904 169.231V198.465C102.904 200.674 104.695 202.465 106.904 202.465C109.113 202.465 110.904 200.674 110.904 198.465V169.231Z"
					fill="#CD24DB"
				/>
			</g>
		</svg>
		<p v-if="isLong">Phamhub</p>
	</div>
</template>

<script setup lang="ts">
withDefaults(
	defineProps<{ size?: number; isLong?: boolean; isLoading?: boolean }>(),
	{
		size: 32,
		isLong: false,
		isLoading: false,
	},
)
</script>

<style lang="css" scoped>
p {
	font-family: Itim;
	@apply text-3xl text-[hsl(var(--secondary-base)_13%)];
}

svg.loading path {
	opacity: 0;
	animation-name: load;
	animation-duration: 1s;
	animation-iteration-count: infinite;
	animation-direction: alternate;
	transform-origin: center;
}

@keyframes load {
	from {
		opacity: 0;
		transform: scale(0.9) translateY(20px);
	}
	80%,
	100% {
		opacity: 1;
		transform: scale(1) translateY(0);
	}
}
</style>

