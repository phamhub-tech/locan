export class KVStore {
	private _getStorage(persisted: boolean): Storage {
		return persisted ? localStorage : sessionStorage
	}

	setItem(key: string, value: any, persist = false) {
		const valueString = JSON.stringify(value)
		const storage = this._getStorage(persist)

		storage.setItem(key, valueString)
	}

	getItem<T = string>(key: string, fromPersisted = false): T | null {
		const storage = this._getStorage(fromPersisted)

		const valueString = storage.getItem(key)
		if (valueString === null) return null

		return JSON.parse(valueString)
	}

	removeItem(key: string, fromPersisted = false) {
		const storage = this._getStorage(fromPersisted)
		return storage.removeItem(key)
	}

	clear(fromPersisted = false) {
		const storage = this._getStorage(fromPersisted)
		storage.clear()
	}
	clearAll() {
		localStorage.clear()
		sessionStorage.clear()
	}
}
