import {TApiStatus } from "~/_common/core/api";
import { delay, getApiMessage } from "~/_common/utils";

import { ProfileModel } from "../_models/profile";

interface IState {
	profileApiStatus: TApiStatus
	profileApiMsg: string
	profile: ProfileModel | null
}

const state = (): IState => ({
	profileApiStatus: TApiStatus.default,
	profileApiMsg: '',
	profile: null
})

export const useProfileStore = defineStore('profile', {
	state,
	actions: {
		async getProfile() {
			if (this.profile) return;

			try {
				this.profileApiStatus = TApiStatus.loading
				this.profileApiMsg = ''

				//const { data: { data } } = await profileService.getProfile()
				await delay(30)
				this.profile = ProfileModel.fromJson({
					id: 1,
					email: 'jane@doe.ac',
					first_name: 'Jane',
					last_name: 'Doe',
					date_joined: new Date().toISOString(),
					avatar: null,
				})

				this.profileApiStatus = TApiStatus.success
			} catch (e) {
				console.error(e)
				this.profileApiStatus = TApiStatus.error
				this.profileApiMsg = getApiMessage(e)
			}
		}
	}
})
