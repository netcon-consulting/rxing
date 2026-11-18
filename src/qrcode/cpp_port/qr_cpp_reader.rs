/*
* Copyright 2016 Nu-book Inc.
* Copyright 2016 ZXing authors
* Copyright 2022 Axel Waggershauser
*/
// SPDX-License-Identifier: Apache-2.0

// Result Reader::decode(const BinaryBitmap& image) const
// {
// #if 1
// 	if (!_hints.isPure())
// 		return FirstOrDefault(decode(image, 1));
// #endif

// 	auto binImg = image.getBitMatrix();
// 	if (binImg == nullptr)
// 		return {};

// 	DetectorResult detectorResult;
// 	if (_hints.hasFormat(BarcodeFormat::QRCode))
// 		detectorResult = DetectPureQR(*binImg);
// 	if (_hints.hasFormat(BarcodeFormat::MicroQRCode) && !detectorResult.isValid())
// 		detectorResult = DetectPureMQR(*binImg);

// 	if (!detectorResult.isValid())
// 		return {};

// 	auto decoderResult = Decode(detectorResult.bits());
// 	auto position = detectorResult.position();

// 	return Result(std::move(decoderResult), std::move(position),
// 				  detectorResult.bits().width() < 21 ? BarcodeFormat::MicroQRCode : BarcodeFormat::QRCode);
// }

// void logFPSet(const FinderPatternSet& fps [[maybe_unused]])
// {
// #ifdef PRINT_DEBUG
// 	auto drawLine = [](PointF a, PointF b) {
// 		int steps = maxAbsComponent(b - a);
// 		PointF dir = bresenhamDirection(PointF(b - a));
// 		for (int i = 0; i < steps; ++i)
// 			log(centered(a + i * dir), 2);
// 	};

// 	drawLine(fps.bl, fps.tl);
// 	drawLine(fps.tl, fps.tr);
// 	drawLine(fps.tr, fps.bl);
// #endif
// }

// Results Reader::decode(const BinaryBitmap& image, int maxSymbols) const
// {
// 	auto binImg = image.getBitMatrix();
// 	if (binImg == nullptr)
// 		return {};

// #ifdef PRINT_DEBUG
// 	LogMatrixWriter lmw(log, *binImg, 5, "qr-log.pnm");
// #endif

// 	auto allFPs = FindFinderPatterns(*binImg, _hints.tryHarder());

// #ifdef PRINT_DEBUG
// 	printf("allFPs: %d\n", Size(allFPs));
// #endif

// 	std::vector<ConcentricPattern> usedFPs;
// 	Results results;

// 	if (_hints.hasFormat(BarcodeFormat::QRCode)) {
// 		auto allFPSets = GenerateFinderPatternSets(allFPs);
// 		for (const auto& fpSet : allFPSets) {
// 			if (Contains(usedFPs, fpSet.bl) || Contains(usedFPs, fpSet.tl) || Contains(usedFPs, fpSet.tr))
// 				continue;

// 			logFPSet(fpSet);

// 			auto detectorResult = SampleQR(*binImg, fpSet);
// 			if (detectorResult.isValid()) {
// 				auto decoderResult = Decode(detectorResult.bits());
// 				auto position = detectorResult.position();
// 				if (decoderResult.isValid()) {
// 					usedFPs.push_back(fpSet.bl);
// 					usedFPs.push_back(fpSet.tl);
// 					usedFPs.push_back(fpSet.tr);
// 				}
// 				if (decoderResult.isValid(_hints.returnErrors())) {
// 					results.emplace_back(std::move(decoderResult), std::move(position), BarcodeFormat::QRCode);
// 					if (maxSymbols && Size(results) == maxSymbols)
// 						break;
// 				}
// 			}
// 		}
// 	}

// 	if (_hints.hasFormat(BarcodeFormat::MicroQRCode) && !(maxSymbols && Size(results) == maxSymbols)) {
// 		for (const auto& fp : allFPs) {
// 			if (Contains(usedFPs, fp))
// 				continue;

// 			auto detectorResult = SampleMQR(*binImg, fp);
// 			if (detectorResult.isValid()) {
// 				auto decoderResult = Decode(detectorResult.bits());
// 				auto position = detectorResult.position();
// 				if (decoderResult.isValid(_hints.returnErrors())) {
// 					results.emplace_back(std::move(decoderResult), std::move(position), BarcodeFormat::MicroQRCode);
// 					if (maxSymbols && Size(results) == maxSymbols)
// 						break;
// 				}

// 			}
// 		}
// 	}

// 	return results;
// }

// } // namespace ZXing::QRCode

use crate::{
    BarcodeFormat, DecodeHints, Exceptions, ImmutableReader, RXingResult, Reader,
    common::{DetectorRXingResult, cpp_essentials::ConcentricPattern},
};

#[cfg(feature = "multi_barcode_readers")]
use crate::multi::MultipleBarcodeReader;

use super::{
    decoder::Decode,
    detector::{
        DetectPureMQR, DetectPureQR, DetectPureRMQR, FindFinderPatterns, GenerateFinderPatternSets,
        SampleMQR, SampleQR, SampleRMQR,
    },
};

#[derive(Default)]
pub struct QrReader;

impl Reader for QrReader {
    fn decode<B: crate::Binarizer>(
        &mut self,
        image: &mut crate::BinaryBitmap<B>,
    ) -> crate::common::Result<crate::RXingResult> {
        self.decode_with_hints(image, &DecodeHints::default())
    }

    fn decode_with_hints<B: crate::Binarizer>(
        &mut self,
        image: &mut crate::BinaryBitmap<B>,
        hints: &DecodeHints,
    ) -> crate::common::Result<RXingResult> {
        self.internal_decode_with_hints(image, hints)
    }
}

impl ImmutableReader for QrReader {
    fn immutable_decode_with_hints<B: crate::Binarizer>(
        &self,
        image: &mut crate::BinaryBitmap<B>,
        hints: &DecodeHints,
    ) -> crate::common::Result<RXingResult> {
        self.internal_decode_with_hints(image, hints)
    }
}

#[cfg(feature = "multi_barcode_readers")]
impl MultipleBarcodeReader for QrReader {
    fn decode_multiple<B: crate::Binarizer>(
        &mut self,
        image: &mut crate::BinaryBitmap<B>,
    ) -> crate::common::Result<Vec<crate::RXingResult>> {
        self.decode_multiple_with_hints(image, &DecodeHints::default())
    }

    fn decode_multiple_with_hints<B: crate::Binarizer>(
        &mut self,
        image: &mut crate::BinaryBitmap<B>,
        hints: &DecodeHints,
    ) -> crate::common::Result<Vec<crate::RXingResult>> {
        self.decode_set_number_with_hints(image, hints, u32::MAX)
    }
}

impl QrReader {
    /// `Reader::decode_with_hints`, with an observer for each symbol the scan
    /// reports.
    ///
    /// The observer is called with the binarized matrix the symbol was sampled
    /// from and the result points, immediately before the symbol is pushed to
    /// the results, so the observer sees exactly the bits `Decode` read its
    /// corrected codewords from and nothing else: one call per reported symbol,
    /// in result order. It is an observation, not a filter — what it returns is
    /// discarded and it cannot change what the scan reports.
    ///
    /// This exists because the `Reader`/`MultipleBarcodeReader` entry points
    /// return the decode results and keep the sampled matrix to themselves. A
    /// consumer that has to look at the same bits the decoder looked at — to
    /// re-read the corrected codewords, or to check what the sampled region
    /// actually contains — cannot do it through those traits, and today has to
    /// copy this loop to get the pairing.
    pub fn decode_with_hints_observed<B: crate::Binarizer>(
        &self,
        image: &mut crate::BinaryBitmap<B>,
        hints: &DecodeHints,
        observer: &mut dyn FnMut(&crate::common::BitMatrix, &[crate::Point]),
    ) -> crate::common::Result<RXingResult> {
        self.internal_decode_with_hints_observed(image, hints, observer)
    }

    /// `MultipleBarcodeReader::decode_multiple_with_hints`, capped at `count`
    /// symbols, with the same observer as [`Self::decode_with_hints_observed`].
    pub fn decode_multiple_with_hints_observed<B: crate::Binarizer>(
        &self,
        image: &mut crate::BinaryBitmap<B>,
        hints: &DecodeHints,
        count: u32,
        observer: &mut dyn FnMut(&crate::common::BitMatrix, &[crate::Point]),
    ) -> crate::common::Result<Vec<RXingResult>> {
        self.decode_set_number_with_hints_observed(image, hints, count, observer)
    }

    fn decode_set_number_with_hints_observed<B: crate::Binarizer>(
        &self,
        image: &mut crate::BinaryBitmap<B>,
        hints: &DecodeHints,
        count: u32,
        observer: &mut dyn FnMut(&crate::common::BitMatrix, &[crate::Point]),
    ) -> crate::common::Result<Vec<RXingResult>> {
        let binImg = image.get_black_matrix(); //image.getBitMatrix();
        let maxSymbols = count;
        // if (binImg == nullptr)
        // 	{return {};}

        // #ifdef PRINT_DEBUG
        // 	LogMatrixWriter lmw(log, *binImg, 5, "qr-log.pnm");
        // #endif
        let try_harder = hints.TryHarder.unwrap_or(false);
        let min_module_size = hints.MinimumModuleSize.unwrap_or(0);

        let mut allFPs = FindFinderPatterns(binImg, try_harder, min_module_size);

        // #ifdef PRINT_DEBUG
        // 	printf("allFPs: %d\n", Size(allFPs));
        // #endif

        let mut usedFPs: Vec<ConcentricPattern> = Vec::new();
        let mut results: Vec<RXingResult> = Vec::new();

        let (check_qr, check_mqr, check_rmqr) = if let Some(formats) = &hints.PossibleFormats {
            (
                formats.contains(&BarcodeFormat::QR_CODE),
                formats.contains(&BarcodeFormat::MICRO_QR_CODE),
                formats.contains(&BarcodeFormat::RECTANGULAR_MICRO_QR_CODE),
            )
        } else {
            (true, true, true)
        };

        if check_qr {
            // if (_hints.hasFormat(BarcodeFormat::QRCode)) {
            let allFPSets = GenerateFinderPatternSets(&mut allFPs);
            for fpSet in allFPSets {
                // for (const auto& fpSet : allFPSets) {
                if usedFPs.contains(&fpSet.bl)
                    || usedFPs.contains(&fpSet.tl)
                    || usedFPs.contains(&fpSet.tr)
                {
                    continue;
                }

                // logFPSet(fpSet);

                let detectorResult = SampleQR(binImg, &fpSet);
                if let Ok(detectorResult) = detectorResult {
                    // if (detectorResult.is_ok()) {
                    let decoderResult = Decode(detectorResult.getBits())
                        .map(|r| r.withCharacterSetHint(hints.CharacterSet.clone()));
                    let position = detectorResult.getPoints();
                    if let Ok(decoderResult) = decoderResult {
                        if decoderResult.isValid() {
                            usedFPs.push(fpSet.bl);
                            usedFPs.push(fpSet.tl);
                            usedFPs.push(fpSet.tr);
                        }

                        if decoderResult.isValid() {
                            // Observe the bits the decoder just read its corrected
                            // codewords from, one call per reported symbol.
                            observer(detectorResult.getBits(), position);
                            results.push(RXingResult::with_decoder_result(
                                decoderResult,
                                position,
                                BarcodeFormat::QR_CODE,
                            ));

                            if maxSymbols != 0 && (results.len() as u32) == maxSymbols {
                                break;
                            }
                        }
                    }
                }
            }
        }
        if check_mqr && !(maxSymbols != 0 && (results.len() as u32) == maxSymbols) {
            // if (_hints.hasFormat(BarcodeFormat::MicroQRCode) && !(maxSymbols && Size(results) == maxSymbols)) {
            for fp in &allFPs {
                // for (const auto& fp : allFPs) {
                if usedFPs.contains(fp) {
                    continue;
                }

                let detectorResult = SampleMQR(binImg, *fp);
                if let Ok(detectorResult) = detectorResult {
                    // if (detectorResult.is_ok()) {
                    let decoderResult = Decode(detectorResult.getBits())
                        .map(|r| r.withCharacterSetHint(hints.CharacterSet.clone()));
                    let position = detectorResult.getPoints();
                    if let Ok(decoderResult) = decoderResult {
                        if decoderResult.isValid() {
                            observer(detectorResult.getBits(), position);
                            results.push(RXingResult::with_decoder_result(
                                decoderResult,
                                position,
                                BarcodeFormat::MICRO_QR_CODE,
                            ));

                            if maxSymbols != 0 && (results.len() as u32) == maxSymbols {
                                break;
                            }
                        }
                    }
                }
            }
        }
        if check_rmqr && !(maxSymbols != 0 && (results.len() as u32) == maxSymbols) {
            for fp in &allFPs {
                // for (const auto& fp : allFPs) {
                if usedFPs.contains(fp) {
                    continue;
                }

                let detectorResult = SampleRMQR(binImg, *fp);
                if let Ok(detectorResult) = detectorResult {
                    // if (detectorResult.is_ok()) {
                    let decoderResult = Decode(detectorResult.getBits())
                        .map(|r| r.withCharacterSetHint(hints.CharacterSet.clone()));
                    let position = detectorResult.getPoints();
                    if let Ok(decoderResult) = decoderResult {
                        if decoderResult.isValid() {
                            observer(detectorResult.getBits(), position);
                            results.push(RXingResult::with_decoder_result(
                                decoderResult,
                                position,
                                BarcodeFormat::RECTANGULAR_MICRO_QR_CODE,
                            ));

                            if maxSymbols != 0 && (results.len() as u32) == maxSymbols {
                                break;
                            }
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    fn decode_set_number_with_hints<B: crate::Binarizer>(
        &self,
        image: &mut crate::BinaryBitmap<B>,
        hints: &DecodeHints,
        count: u32,
    ) -> crate::common::Result<Vec<RXingResult>> {
        self.decode_set_number_with_hints_observed(image, hints, count, &mut |_, _| {})
    }

    fn internal_decode_with_hints<B: crate::Binarizer>(
        &self,
        image: &mut crate::BinaryBitmap<B>,
        hints: &DecodeHints,
    ) -> crate::common::Result<RXingResult> {
        self.internal_decode_with_hints_observed(image, hints, &mut |_, _| {})
    }

    fn internal_decode_with_hints_observed<B: crate::Binarizer>(
        &self,
        image: &mut crate::BinaryBitmap<B>,
        hints: &DecodeHints,
        observer: &mut dyn FnMut(&crate::common::BitMatrix, &[crate::Point]),
    ) -> crate::common::Result<RXingResult> {
        // #if 1
        if !matches!(hints.PureBarcode, Some(true))
        // if !matches!(Some(hints.get(&DecodeHintType::PURE_BARCODE)))
        // if (!_hints.isPure())
        {
            return Ok(self
                .decode_set_number_with_hints_observed(image, hints, 1, observer)?
                .first()
                .ok_or(Exceptions::NOT_FOUND)?
                .clone());
            // return FirstOrDefault(decode(image, 1));
        }
        // #endif

        let binImg = image.get_black_matrix(); //image.getBitMatrix();
        // if (binImg == nullptr)
        // 	{return {};}

        let mut detectorResult = Err(Exceptions::NOT_FOUND);
        if let Some(formats) = &hints.PossibleFormats {
            if formats.contains(&BarcodeFormat::QR_CODE) {
                detectorResult = DetectPureQR(binImg);
            }
            if formats.contains(&BarcodeFormat::MICRO_QR_CODE) && detectorResult.is_err() {
                detectorResult = DetectPureMQR(binImg);
            }
            if formats.contains(&BarcodeFormat::RECTANGULAR_MICRO_QR_CODE)
                && detectorResult.is_err()
            {
                detectorResult = DetectPureRMQR(binImg);
            }
        }

        if detectorResult.is_err() {
            for decode_function in [DetectPureQR, DetectPureMQR, DetectPureRMQR] {
                detectorResult = decode_function(binImg);
                if detectorResult.is_ok() {
                    break;
                }
            }
        }

        let detectorResult = detectorResult?;

        // let detectorResult: DetectorResult;
        // if (_hints.hasFormat(BarcodeFormat::QR_CODE))
        // 	{detectorResult = DetectPureQR(binImg);}

        // if (_hints.hasFormat(BarcodeFormat::MICRO_QR_CODE) && !detectorResult.isValid())
        // 	{detectorResult = DetectPureMQR(binImg);}

        // if (!detectorResult.isValid())
        // 	{return {};}

        let decoderResult =
            Decode(detectorResult.getBits())?.withCharacterSetHint(hints.CharacterSet.clone());
        let position = detectorResult.getPoints();

        observer(detectorResult.getBits(), position);
        Ok(RXingResult::with_decoder_result(
            decoderResult,
            position,
            if detectorResult.getBits().width() != detectorResult.getBits().height() {
                BarcodeFormat::RECTANGULAR_MICRO_QR_CODE
            } else if detectorResult.getBits().width() < 21 {
                BarcodeFormat::MICRO_QR_CODE
            } else {
                BarcodeFormat::QR_CODE
            },
        ))
    }
}
