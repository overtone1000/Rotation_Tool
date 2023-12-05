import { writable, type Unsubscriber } from 'svelte/store';
import { StagingSelectionMode } from '../../commons/refactored/staging/members/highlighting';
import { getStagingHighlightKeyframes } from '../../commons/refactored/theming/theme';
import { AnimationConstants } from './animation';

export class HighlightingStoreProvider {
	private highlight_color = writable<StagingSelectionMode>(StagingSelectionMode.none);
	public getHighlightColorStore() {
		return this.highlight_color;
	}
	public updateHighlighting(highlighting: StagingSelectionMode): void {
		this.highlight_color.update(() => {
			return highlighting;
		});
	}
}

export class HighlightingStore {
	private highlighting_store_subscription: Unsubscriber | undefined;
	public highlight_element: HTMLElement | undefined;
	private border_animation: Animation | undefined;

	public update(provider: HighlightingStoreProvider | undefined) {
		if (provider) {
			this.unsubscribe();
			const highlighting_store = provider.getHighlightColorStore();
			highlighting_store.subscribe((highlighting) => {
				if (this.highlight_element !== undefined) {
					this.updateBorderAnimation(highlighting);
				}
			});
		}
	}

	private updateBorderAnimation(highlighting: StagingSelectionMode) {
		if (highlighting === undefined || highlighting == StagingSelectionMode.none) {
			if (this.border_animation) {
				this.border_animation.cancel();
			}
		} else {
			if (this.border_animation) {
				this.border_animation.cancel();
			}
			if (this.highlight_element) {
				const keyframes = getStagingHighlightKeyframes(highlighting);
				if (keyframes) {
					//console.debug("Starting animation.");
					this.border_animation = this.highlight_element.animate(
						keyframes,
						AnimationConstants.options
					);
					AnimationConstants.syncHighlight(this.border_animation);
				}
			}
		}
	}

	public unsubscribe() {
		if (this.highlighting_store_subscription) {
			this.highlighting_store_subscription();
		}
	}
}
