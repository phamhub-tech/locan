import type { AxiosError } from "axios";
import { clsx, type ClassValue } from "clsx";
import {
	differenceInDays,
	differenceInHours,
	format,
	formatDistance,
	formatRelative,
} from "date-fns";
import { enGB } from "date-fns/locale/en-GB";
import { enUS } from "date-fns/locale/en-US";
import { twMerge } from "tailwind-merge";
import type { HTMLAttributes } from "vue";

import type { IApiResponse } from "~/_common/core/api";

import type { IColor, TColorString } from "../types";

export * from './auth';

export function abbreviateCount(count: number): string {
	let suffix = ''
	let dividend = 1
	if (count >= 1_000_000) {
		suffix = 'M'
		dividend = 1_000_000
	} else if (count >= 1_000) {
		suffix = 'K'
		dividend = 1_000
	}

	let num = (count / dividend).toFixed(1)
	console.log('Num', num)
	if (num[num.length - 1] === '0') num = num.substring(0, num.length - 2)

	console.log('C: ', count, num, suffix)
	return `${num}${suffix}`
}

/**
 * Debounces an action
 */
export class Debouncer {
	private _timer: number | null = null;
	constructor(private _timeout = 800) { }

	/**
	 * Debounces the `callback` that is passed in
	 */
	run(callback: () => void) {
		if (this._timer !== null) window.clearTimeout(this._timer)

		this._timer = window.setTimeout(callback, this._timeout)
	}
}

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

/**
 * Converts `IColor` to hsl string
 *
 * @param color The color to transform
 * @param other Parts of color that should be replaced with new values
 */
export function colorToHsl(
	color: IColor,
	other?: { hue?: string; sat?: string; lum?: string; opacity?: string }
) {
	const { hue: h, sat: s, lum: l, opacity } = other ?? {};
	const { hue, sat, lum } = color;
	return `hsl(${h ?? hue}, ${s ?? sat}, ${l ?? lum}, ${opacity ?? 1})`;
}

/**
 * Returns a comma delimited version of the given number
 *
 * @param num The number to delimit
 */
export function delimit(num: number): string {
	const numStr = num.toString();
	return numStr.replace(/\B(?=(\d{3})+(?!\d))/g, ",");
}

/**
 * Returns an appropriate error message from a failed api request
 *
 * @param err - The error object
 * @param message - An optional error message
 */
export function getApiMessage(
	err: any,
	message = 'An error occured. Please try again later.',
): string {
	let msg = message
	// TODO: Implement localization for default error messages

	const errCode = (err as AxiosError).code
	if (errCode && ['ERR_NETWORK', 'ECONNABORTED'].includes(errCode))
		msg =
			'Could not connect to our the servers. Make sure you have an internet connection and try again.'

	const resp: IApiResponse<any> = (err as any).response
	if (resp) msg = resp.data.message

	if (msg === message) console.error(err)
	return msg
}

/**
 * Gets the percentage of the given number
 *
 * @param number The number
 * @param total The total
 */
export function getPercentage(number: number, total: number): string {
	const perc = (number / total) * 100;

	let percStr = perc.toString();
	if (percStr.includes(".")) {
		percStr = perc.toFixed(1);
		const last = percStr[percStr.length - 1];
		if (last === "0") percStr = percStr.substring(0, percStr.length - 2);
	}

	return percStr;
}

/**
 * Gets the full route name while including the right locale
 *
 * @param name The base route name
 */
export function getRouteName(name: string): string {
	return `${name}___en`
}

/**
 * Converts a 6-digit hex string to hsl
 *
 * @param hex The color in 6-digit hex
 */
export function hexToHsl(hex: string): { hue: number, sat: number, lum: number } {
	hex = hex.replace(/^#/, '');

	// Parse the hex string into RGB components
	const r = parseInt(hex.substring(0, 2), 16) / 255;
	const g = parseInt(hex.substring(2, 4), 16) / 255;
	const b = parseInt(hex.substring(4, 6), 16) / 255;

	// Find the min and max values of RGB
	const max = Math.max(r, g, b);
	const min = Math.min(r, g, b);
	let h = 0,
		s = 0,
		l = (max + min) / 2;

	if (max === min) {
		// Achromatic (no hue)
		h = s = 0;
	} else {
		const d = max - min;
		s = l > 0.5 ? d / (2 - max - min) : d / (max + min);

		switch (max) {
			case r:
				h = (g - b) / d + (g < b ? 6 : 0);
				break;
			case g:
				h = (b - r) / d + 2;
				break;
			case b:
				h = (r - g) / d + 4;
				break;
		}

		h /= 6;
	}

	// Convert hue to degrees, saturation and lightness to percentages
	return {
		hue: Math.round(h * 360),
		sat: Math.round(s * 100),
		lum: Math.round(l * 100),
	};
}

/**
 * Returns a human friendly version of the date
 *
 * @param date The date to humanize
 */
export function humanizeDate(date: Date, addRelTime = true): string {
	const now = new Date();
	const localeToUse = enUS;

	const daysDiff = differenceInDays(now, date);
	// console.log(date.toISOString(), daysDiff, daysDiff < 2)
	if (daysDiff < 1) {
		let parsedDate = formatRelative(date, now, { locale: localeToUse }).replace(
			/^\w/,
			(v) => v.toUpperCase()
		);
		if (addRelTime && differenceInHours(now, date) <= 6) {
			const relString = formatDistance(date, now, {
				addSuffix: true,
				locale: localeToUse,
			}).replace(/^\w/, (v) => v.toUpperCase());
			parsedDate += ` (${relString})`;
		}

		return parsedDate;
	} else if (daysDiff < 7) {
		return format(date, "eeee 'At' h:mm a");
	} else return format(date, "MMM do, yyyy", { locale: localeToUse });
}

type TAttributes = keyof HTMLAttributes | Omit<string, keyof HTMLAttributes>
export function removeFromAttrs(
	attrs: Record<string, any>,
	attr: TAttributes | TAttributes[],
): Record<string, any> {
	const at: Record<string, any> = {}
	for (const attrib of Object.keys(attrs)) {
		if (Array.isArray(attr) && attr.includes(attrib)) continue
		else if (attrib === attr) continue

		at[attrib] = attrs[attrib]
	}

	return at
}

export async function delay(
	timeout: number,
	callback?: () => void | Promise<void>
): Promise<void> {
	return new Promise((res) =>
		setTimeout(async () => {
			if (callback) await callback();
			res();
		}, timeout)
	);
}

/**
 * Takes a color string and converts it to a color
 *
 * @param color The string to convert
 */
export function stringToColor(color: TColorString): IColor {
	const [hue, sat, lum] = color.split(',').map((c) => c.trim())
	return { hue, sat, lum }
}
