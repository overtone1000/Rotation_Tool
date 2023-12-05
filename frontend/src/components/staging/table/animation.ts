export class AnimationConstants {
	public static options: KeyframeAnimationOptions = {
		duration: 1200,
		iterations: Infinity,
		direction: 'alternate'
	};

	private static first_animation: Animation | undefined = undefined;
	public static syncHighlight = (animation_to_sync: Animation) => {
		if (
			AnimationConstants.first_animation === undefined ||
			AnimationConstants.first_animation.playState != 'running'
		) {
			animation_to_sync.startTime = 0;
			AnimationConstants.first_animation = animation_to_sync;
		} else {
			animation_to_sync.startTime = AnimationConstants.first_animation.startTime;
		}
	};
}
