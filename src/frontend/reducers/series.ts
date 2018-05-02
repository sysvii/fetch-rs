import { route } from "preact-router";
import * as actions from "../actions";
import * as api from "../api";
import store from "../store";

const INITAL_SATE = {
  items: [],
  loading: false,
};

const seriesReducer = (state, action) => {
  if (!state) {
    return INITAL_SATE;
  }

  switch (action.type) {
    case "GET_ALL_SERIES":
      return { ...state, ...getAllSeries() };
    case "FINISHED_GET_ALL_SERIES":
      return { ...state, loading: false, items: action.series };

    case "GET_SERIES":
      return { ...state, ...getSeries(parseInt(action.id, 10), state.items) };

    case "FINISHED_GET_SERIES":
      const seriesList: ISeries[] = state.items.filter((ele) => ele.id !== parseInt(action.id, 10));
      if (action.series) {
        seriesList.push(action.series);
      }
      return { ...state, loading: false, items: seriesList };

    case "UPSERT_SERIES":
      return { ...state, ...upsertSeries(action.formData) };

    default:
      return state;
  }
};

const getAllSeries = () => {
  api.getSeries()
    .then((series) => {
      store.dispatch(actions.finishedGetAllSeries(series));
    }).catch((err) => {
      store.dispatch(actions.showError(err.toString()));
      store.dispatch(actions.finishedGetAllSeries([]));
    });
  return { loading: true };
};

const getSeries = (id: number, stateSeries) => {
  if (stateSeries.find((ele) => ele.id === id)) {
    return { loading: false };
  }

  api.getSeriesId(id).then((series) => {
    store.dispatch(actions.finishedGetSeries(id, series));
  }).catch((err) => {
    store.dispatch(actions.showError(err.toString()));
    store.dispatch(actions.finishedGetSeries(id, null));
  });
  return { loading: true };
};

const upsertSeries = (formData: SeriesFull) => {

  api.upsertSeries(formData)
    .then((series) => {
      store.dispatch(actions.finishedGetSeries(series.id, series));
      store.dispatch(actions.clearInfoBlob(series.id));
      route(`/series/${series.id}`);
    }).catch((err) => {
      store.dispatch(actions.showError(err.toString()));
    });

  return { loading: true };
};

export default seriesReducer;