import { HttpClient, HttpParams } from '@angular/common/http';
import { Injectable, inject } from '@angular/core';
import { map, Observable } from 'rxjs';
import { environment } from '../../environments/environment';
import { PrefsResponseData, PrefValueResponseData, ScopesResponseData } from '../models/http-responses';
import { PrefEntry } from '../models/pref-entry.interface';

@Injectable({
    providedIn: 'root',
})
export class PrefsService {
    private _http = inject(HttpClient)

    public getScopes(): Observable<string[]> {
        return this._http.get<ScopesResponseData>(`${environment.baseUrl}/api/v1/prefs/scopes`)
            .pipe(map((data) => data.scopes))
    }

    public getPrefsInScope(scope: string): Observable<PrefEntry[]> {
        return this._http.get<PrefsResponseData>(`${environment.baseUrl}/api/v1/prefs/${scope}`)
            .pipe(map((data) => data.prefs))
    }

    public getPref(scope: string, key: string): Observable<string | null> {
        return this._http.get<PrefValueResponseData>(`${environment.baseUrl}/api/v1/prefs/${scope}/${key}`)
            .pipe(map((data) => data.value))
    }

    public createPref(scope: string, key: string, value: string | null): Observable<{}> {
        let form = new HttpParams()
        if (value != null) {
            form = form.set('value', value)
        }
        return this._http.post(`${environment.baseUrl}/api/v1/prefs/${scope}/${key}`, form)
    }

    public updatePref(scope: string, key: string, value: string | null): Observable<{}> {
        let form = new HttpParams()
        if (value != null) {
            form = form.set('value', value)
        }
        return this._http.put(`${environment.baseUrl}/api/v1/prefs/${scope}/${key}`, form)
    }

    public deletePref(scope: string, key: string): Observable<{}> {
        return this._http.delete(`${environment.baseUrl}/api/v1/prefs/${scope}/${key}`)
    }
}
