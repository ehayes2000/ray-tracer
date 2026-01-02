// @ts-check

// adapted from
// https://www.pbr-book.org/3ed-2018/Primitives_and_Intersection_Acceleration/Bounding_Volume_Hierarchies#BVHAccel::primitives

const MAX = 10000;
const MIN = -MAX;
/**
 * @typedef {[number, number]} Pt
*/

/**
 * @typedef {object} Ray
 * @property {Pt} origin
 * @property {Pt} direction
 */

/**
 *           [tr, bl]
 * @typedef {[Pt, Pt]} Bounds
 */

/**
 * @typedef {object} PrimitiveInfo
 * @property {Bounds} boundingBox
 * @property {Pt} boundingBoxCentroid
 * @property {Primitive} primitive
 */

/**
 * @typedef {object} Triangle
 * @property {Pt} a
 * @property {Pt} b
 * @property {Pt} c
 */

/**
 * @typedef {Triangle} Primitive
 */

/**
 * @typedef {object} InteriorNode
 * @property {BvhNode} lu
 * @property {BvhNode} rd
 * @property {"x" | "y"} splitAxis
 * @property {Bounds} bounds
 * @property {number} nPrimitives
 */

/**
 * @typedef {(PrimitiveInfo | InteriorNode)} BvhNode
 */

/**
 *
 * @param {Primitive} primitive
 * @returns {PrimitiveInfo}
 */
function primitiveInfo(primitive) {
	/** @type {Pt} */
	const tl = [
		Math.min(primitive.a[0], primitive.b[0], primitive.c[0]),
		Math.min(primitive.a[1], primitive.b[1], primitive.c[1]),
	];

	/** @type {Pt} */
	const br = [
		Math.max(primitive.a[0], primitive.b[0], primitive.c[0]),
		Math.max(primitive.a[1], primitive.b[1], primitive.c[1]),
	];

	const centroid = center([tl, br]);

	return {
		boundingBox: [tl, br],
		boundingBoxCentroid: centroid,
		primitive,
	};
}

/**
 *
 * @param {Bounds} bounds
 * @returns {Pt}
 */
function center(bounds) {
  const [tl, br] = bounds;
  return [
    tl[0] + (br[0] - tl[0]) / 2,
    tl[1] + (br[1] - tl[1]) / 2];
}

/**
 * @returns {Bounds}
 */
function smallBounds() {
  return [[MAX, MAX], [MIN, MIN]];
}


/**
 * @param {[any, any]} pt
 * @returns {pt is Pt}
 */
function isPt(pt){
  return typeof pt[0] === "number" && typeof pt[1] === "number";
}

/**
 *
 * @param {Bounds} a
 * @param {Bounds | Pt} b
 * @returns {Bounds}
 */
function union(a, b) {
  if (isPt(b)) {
    return [
      [Math.min(a[0][0], b[0]), Math.min(a[0][1], b[1])],
      [Math.max(a[1][0], b[0]), Math.max(a[1][1], b[1])]
    ];
  }
  return [
    [Math.min(a[0][0], b[0][0]), Math.min(a[0][1], b[0][1])],
    [Math.max(a[1][0], b[1][0]), Math.max(a[1][1], b[1][1])]
  ];
}

/**
 *
 * @param {Bounds} bounds
 * @param {Pt} pt
 * @returns {Pt}
 */
function relativeToBounds(bounds, pt) {
  /** @type {Pt} */
  const o = [pt[0] - bounds[0][0], pt[1] - bounds[0][1]];
  if (bounds[1][0] > bounds[0][0]) o[0] /= bounds[1][0] - bounds[0][0];
  if (bounds[1][1] > bounds[0][1]) o[1] /= bounds[1][1] - bounds[0][1];
  return o;
}

/**
 * @param {Array<PrimitiveInfo>} primitives
 * @returns {BvhNode}
*/
function recursiveBuild(primitives) {
  if (primitives.length === 1){
    return primitives[0]
  }
  const centroidBounds = primitives
    .reduce(
      (bounds, info) => union(bounds, info.boundingBoxCentroid),
      smallBounds());
  const bounds = primitives.reduce((bounds, info) => union(bounds, info.boundingBox), smallBounds())

  const partition = centroidBounds[1][0] - centroidBounds[0][0] > centroidBounds[1][1] - centroidBounds[0][1] ? "x" : "y";
  const centroid = center(centroidBounds);

  /**
  * @param {PrimitiveInfo} p
  */
  const isLeftAbove = (p) => {
    if (partition === "x") {
      return p.boundingBoxCentroid[0] < centroid[0];
    } else {
      return p.boundingBoxCentroid[1] < centroid[1];
    }
  }

  const [luPrimitives, rdPrimitives] = primitives.reduce(([l, r], p) => {
    if (isLeftAbove(p)) {
      return [[...l, p], r];
    } else {
      return [l, [...r, p]];
    }
  }, /** @type {[Array<PrimitiveInfo>, Array<PrimitiveInfo>]}*/([[], []]));

  const lu = recursiveBuild(luPrimitives);
  const rd = recursiveBuild(rdPrimitives);

  return {
    bounds,
    lu,
    rd,
    splitAxis: partition,
    nPrimitives: primitives.length
  }
}


/**
 *
 * @param {Bounds} bounds
 * @returns {0 | 1}
 */
function maximumExtend(bounds) {
  const w = bounds[1][0] - bounds[0][0];
  const h = bounds[1][1] - bounds[0][1];
  return w > h ? 0 : 1
}

/**
 * @param {Bounds} bounds
 * @returns {number}
 */
function surfaceArea(bounds){
  const w = bounds[1][0] - bounds[0][0];
  const h = bounds[1][1] - bounds[0][1];
  return w * h;
}

const N_BUCKETS = 12;

/**
* @typedef {object} BucketInfo
* @property {number} count
* @property {Bounds} bounds
*
*/

/**
 * @returns {BucketInfo}
 */
function emptyBucket(){
  return {
    bounds: smallBounds(),
    count: 0
  }
}

/**
 *
 * @param {Bounds} bounds
 * @param {Ray} ray
 */
function intersect(bounds, ray) {
  let t0 = 0;
  let t1 = 0;
  for (let i = 0; i < 2; i++) {
    const invRayDir = 1 / ray.direction[0];
    let tNear = (bounds[0][0] - ray.origin[0]) * invRayDir;
    let tFar = (bounds[1][0] - ray.origin[0]) * invRayDir;
    if (tNear > tFar) {
      [tNear, tFar] = [tFar, tNear];
    }
    t0 = Math.max(tNear, t0);
    t1 = Math.min(tFar, t1);
    if (t0 > t1) return;
  }
  return {
    t0,
    t1
  }
}

 /**
  *
  * @param {Bounds} bounds
  * @param {Pt} pt
  * @returns {boolean}
  */
function contains(bounds, pt) {
   return (pt[0] >= bounds[0][0]
     && pt[0] <= bounds[1][0]
     && pt[1] >= bounds[0][1]
     && pt[1] <= bounds[1][1]
   );
}

/**
 *
 * @param {BvhNode} node
 * @returns {node is PrimitiveInfo}
 */
function isLeafNode(node){
  return !("bounds" in node)
}

/**
 *
 * @param {BvhNode} root
 * @param {Pt} pt
 * @returns {[BvhNode[], number]}
 */
function searchBvh(root, pt) {
	if (isLeafNode(root) && contains(root.boundingBox, pt)) {
		return [[root], 1];
	} else if (isLeafNode(root)) {
		return [[], 1];
	} else if (!contains(root.bounds, pt)) return [[], 1];
	const [lu, lcount] = searchBvh(root.lu, pt);
	const [rd, rcount] = searchBvh(root.rd, pt);
	return [[...lu, ...rd, root], lcount + rcount + 1];
}

 /**
 *
 * @param {Triangle} t
 * @returns {Path2D}
 */
function triangleToPath(t) {
  const path = new Path2D();
  path.moveTo(...t.a);
  path.lineTo(...t.b);
  path.lineTo(...t.c);
  return path;
}

/// _____ Run _____
const canvas = document.createElement("canvas");
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;
document.body.appendChild(canvas);

const ctx = /** @type {CanvasRenderingContext2D}*/ (canvas.getContext("2d"));

if (!ctx) {
  throw new Error("No render context");
}

/**
*
* @param {Triangle} t
*/
function drawTriangle(t) {
  const path = triangleToPath(t)
  ctx.fill(path)
}

const size = 5;
const n = 2048;

/** @type {Array<Triangle>} */
const triangles = [];
for (let i = 0; i < n; i++) {
  const [x, y] = [
    Math.random() * canvas.clientWidth, Math.random() * canvas.clientHeight
  ];
  /** @type {Pt} */
  const a = [x - size, y + size];
  /** @type {Pt} */
  const b = [x + size, y + size];
  /** @type {Pt} */
  const c = [x, y - size];
  triangles.push({
    a, b, c
  });
}

function drawTriangles() {
  ctx.clearRect(0, 0, canvas.width,canvas.height)
  ctx.fillStyle = "white";
  for (const t of triangles) {
    drawTriangle(t)
  }
}

/**
 *
 * @param {BvhNode} node
 */
function drawBvh(node) {
  if (!node || isLeafNode(node)) return;
  hightlightBvh(node);
  drawBvh(node.lu)
  drawBvh(node.rd)
}

drawTriangles()
const info = triangles.map(primitiveInfo)
const bvh = recursiveBuild(info)

/**
 * @param {BvhNode} node
*/
function hightlightBvh(node) {
  const bounds = isLeafNode(node) ? node.boundingBox : node.bounds;
  /** @type {[number, number, number, number]} */
  const args =[
      bounds[0][0],
			bounds[0][1],
			bounds[1][0] - bounds[0][0],
			bounds[1][1] - bounds[0][1],
		]
  ctx.strokeStyle = "springgreen";
  ctx.strokeRect(...args);
  if (isLeafNode(node)) {
    const path = triangleToPath(node.primitive);
    ctx.fillStyle = "springgreen";
    ctx.fill(path);
  }
}

const interactive = true;
let searches = 0;
let totalDepth = 0;

if (interactive) {
  canvas.addEventListener("mousemove", (e) => {
    drawTriangles();
    const [nodes, count]= searchBvh(bvh, [e.clientX, e.clientY]);
    searches++;
    totalDepth += count;
    console.log("avg search depth", totalDepth / searches )
    if (!nodes) return;
    for (const n of nodes) {
      hightlightBvh(n);
    }
  })
}
else {
  drawBvh(bvh)
}
